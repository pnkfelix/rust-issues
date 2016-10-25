extern crate ipc_channel;
extern crate net_traits;

pub mod http_loader {
    use ipc_channel::ipc::{channel, IpcReceiver, IpcSender};
    use net_traits::{CustomResponse};

    #[derive(Debug)]
    pub struct LoadError;

    #[inline(always)]
    pub fn load<A>(_request_factory: A) -> Result<(), LoadError>
    {
        let (_msg_sender, msg_receiver)
            : (IpcSender<Option<CustomResponse>>, IpcReceiver<Option<CustomResponse>>)
            = channel().unwrap();
        msg_receiver.recv().unwrap();

        unimplemented!()
    }
}
