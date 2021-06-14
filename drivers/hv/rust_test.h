
void print_hello(void);
u32 rust_hv_get_next_write_location(struct hv_ring_buffer_info *ring_info);
u32 hv_get_next_write_location(struct hv_ring_buffer_info *ring_info);
void hv_set_next_write_location(struct hv_ring_buffer_info *ring_info, u32 next_write_location);
void hv_set_next_read_location(struct hv_ring_buffer_info *ring_info, u32 next_read_location);
//void hv_signal_on_write(u32 old_write, struct vmbus_channel *channel);