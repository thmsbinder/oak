initSidebarItems({"enum":[["ChannelEither","A wrapper type to allow taking channel references without discriminating on direction. Used when adding `ChannelRef`s into `Message`s."],["ReadStatus","A helper type to determine if `try_read_message` was called with not enough `bytes_capacity` and/or `handles_capacity`."]],"fn":[["new","Creates a new `ChannelWriter` and `ChannelReader` that reference the same underlying `Channel`."],["readers_statuses","Reads the statuses from a slice of `Option<&ChannelReader>`s. `ChannelReadStatus::INVALID_CHANNEL` is set for `None` readers in the slice. For `Some(_)` readers, the result is set from a call to `has_message`."],["wait_on_channels","Waits on a slice of `Option<&ChannelReader>`s, blocking until one of the following conditions: - If the `Runtime` is terminating this will return immediately with an `ERR_TERMINATED` status   for each channel. - If all readers are in an erroneous status, e.g. when all `ChannelReaders` are orphaned, this   will immediately return the channels statuses. - If any of the channels is able to read a message, the corresponding element in the returned   vector will be set to `Ok(READ_READY)`, with `Ok(NOT_READY)` signaling the channel has no   message available"]],"struct":[["Channel","The internal implementation of a channel representation backed by a `VecDeque<Message>`. Readers and writers to this channel must increment the reader/writer count. This is implemented for `ChannelWriter`/`ChannelReader`, which will increment/decrement readers/writers when cloning/dropping."],["ChannelReader","Reader end to a `Channel`. `ChannelReader` implements `Clone` and `Drop` to automatically update the underlying channel."],["ChannelWriter","Writer end to a `Channel`. `ChannelWriter` implements `Clone` and `Drop` to automatically update the underlying channel."]]});