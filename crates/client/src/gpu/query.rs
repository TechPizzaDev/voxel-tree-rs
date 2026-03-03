use std::ops::{Deref, Range};

use wgpu::{Buffer, BufferUsages, CommandEncoder, QuerySet};

use crate::{
    app::FrameIndex,
    sync::mpsc::{Channel, TryRecvError},
};

#[derive(Debug)]
pub enum SubmitError {
    BufferPoolExhausted(TryRecvError),
}

#[derive(Debug)]
pub enum PollError {
    Empty,
    BufferPool(FrameIndex),
    BufferMap(FrameIndex),
    ResultDisconnected,
}

pub struct QueryInfo {
    query_range: Range<u32>,
    query_set: QuerySet,
    resolve_buffer: Buffer,
    pool_size: u32,
    buf_pool: Channel<QueryBuffer>,
    results: Channel<Result<QueryResult, FrameIndex>>,
}
impl QueryInfo {
    pub fn new(range: Range<u32>, device: &wgpu::Device) -> Self {
        let count = range.len() as u32;
        let query_set = device.create_query_set(&wgpu::QuerySetDescriptor {
            label: None,
            ty: wgpu::QueryType::Timestamp,
            count,
        });
        let resolve_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: Self::to_query_size(&range),
            usage: BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self {
            query_range: range,
            query_set,
            resolve_buffer,
            pool_size: 0,
            buf_pool: Channel::default(),
            results: Channel::default(),
        }
    }

    pub fn query_set(&self) -> &QuerySet {
        &self.query_set
    }

    pub fn to_query_size(range: &Range<u32>) -> u64 {
        range.len() as u64 * wgpu::QUERY_SIZE as u64
    }

    fn new_buffer(&mut self, device: &wgpu::Device) -> QueryBuffer {
        let index = self.pool_size;
        self.pool_size = index.strict_add(1);

        QueryBuffer::new(
            index,
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("Query Result {}", index)),
                size: Self::to_query_size(&self.query_range),
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
        )
    }

    pub fn add_buffer(&mut self, device: &wgpu::Device) {
        let first_buf = self.new_buffer(device);
        self.buf_pool.send(first_buf).unwrap();
    }

    pub fn poll(&self, output: &mut Vec<u8>) -> Result<FrameIndex, PollError> {
        let query = match self.results.try_recv() {
            Ok(result) => result.map_err(PollError::BufferMap),
            Err(TryRecvError::Disconnected) => Err(PollError::ResultDisconnected),
            Err(TryRecvError::Empty) => Err(PollError::Empty),
        }?;

        let view = query.buffer.get_mapped_range(..);
        output.extend_from_slice(&view);

        drop(view);
        query.buffer.unmap();
        
        self.buf_pool
            .send(query.buffer)
            .map_err(|_| PollError::BufferPool(query.frame))?;
        Ok(query.frame)
    }

    pub fn submit(
        &mut self,
        encoder: &mut CommandEncoder,
        frame: FrameIndex,
    ) -> Result<(), SubmitError> {
        let buffer = self
            .buf_pool
            .try_recv()
            .map_err(SubmitError::BufferPoolExhausted)?;

        encoder.resolve_query_set(
            &self.query_set,
            self.query_range.clone(),
            &self.resolve_buffer,
            0,
        );

        let copy_size = wgpu::QUERY_SIZE as u64 * self.query_range.len() as u64;
        encoder.copy_buffer_to_buffer(&self.resolve_buffer, 0, &buffer.inner, 0, copy_size);

        let result_sender = self.results.sender().clone();

        encoder.map_buffer_on_submit(
            &buffer.inner.clone(),
            wgpu::MapMode::Read,
            ..,
            move |result| {
                let send_result = result_sender.send(match result {
                    Ok(()) => Ok(QueryResult { frame, buffer }),
                    Err(_) => Err(frame),
                });
                // Avoid panic-ing wgpu caller.
                // TODO: send error somewhere?
                drop(send_result);
            },
        );
        Ok(())
    }
}

struct QueryBuffer {
    index: u32,
    inner: Buffer,
}
impl QueryBuffer {
    fn new(index: u32, inner: Buffer) -> Self {
        Self { index, inner }
    }
}
impl Deref for QueryBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

struct QueryResult {
    frame: FrameIndex,
    buffer: QueryBuffer,
}
