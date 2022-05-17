use std::net::SocketAddr;

use rust_raknet::*;
use pyo3::prelude::*;
use lazy_static::*;
use pyo3::exceptions::*;

lazy_static!{
    static ref RT : tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
}

#[pyclass(name = "RaknetServer")]
pub struct RaknetServer {
    server : RaknetListener
}

#[pyclass(name = "RaknetClient")]
pub struct RaknetClient {
    client : RaknetSocket
}

#[pyfunction]
pub fn enablelog(){
    enable_raknet_log(255);
}

#[pyfunction]
pub fn ping(address : &PyAny) -> PyResult<String> {
    let address = address.str().unwrap().to_string();
    let (_ , motd) = match RT.block_on(RaknetSocket::ping(&address.parse().unwrap())){
        Ok(p) => p,
        Err(_) => {
            return Err(PyBaseException::new_err("address parse error"));
        }
    };
    Ok(motd)
}

#[pymethods]
impl RaknetClient{

    #[new]
    pub fn new(address : String ) -> PyResult<RaknetClient> {
        let address : SocketAddr = match address.parse(){
            Ok(p) => p,
            Err(_) => {
                return Err(PyBaseException::new_err("address parse error"));
            }
        };
        let client = match match RT.block_on(async {
            tokio::time::timeout(std::time::Duration::from_secs(10) , RaknetSocket::connect(&address)).await
        }){
            Ok(p) => p,
            Err(_) => {
                return Err(PyBaseException::new_err("connect timeout"));
            }
        }{
            Ok(p) => p,
            Err(_) => {
                return Err(PyBaseException::new_err("connect error"));
            }
        };

        Ok(RaknetClient{
            client : client
        })
    }

    pub fn send(&mut self ,buf : Vec::<u8>) -> PyResult<()> {
        match RT.block_on(self.client.send(&buf , Reliability::ReliableOrdered)){
            Ok(_) => {},
            Err(_) => {
                return Err(PyBaseException::new_err("send error"));
            }
        };
        Ok(())
    }

    pub fn recv(&mut self) -> PyResult<Vec<u8>>{
        let buf = match RT.block_on(self.client.recv()){
            Ok(p) => p,
            Err(_) => {
                return Err(PyBaseException::new_err("recv error"));
            }
        };

        Ok(buf.into())
    }

    pub fn peeraddr(&mut self) -> PyResult<String> {
        Ok(self.client.peer_addr().unwrap().to_string())
    }

    pub fn localaddr(&mut self) -> PyResult<String> {
        Ok(self.client.local_addr().unwrap().to_string())
    }

    pub fn close(&mut self) -> PyResult<()> {
        RT.block_on(self.client.close()).unwrap();
        Ok(())
    }
}

#[pymethods]
impl RaknetServer {

    #[new]
    pub fn new(address : &PyAny ) -> PyResult<RaknetServer>{
        let address = address.str()?;
        let address = address.to_string();
        let address : SocketAddr = match address.parse(){
            Ok(p) => p,
            Err(_) => {
                return Err(PyBaseException::new_err("address parse error"));
            }
        };
        let mut server = RT.block_on(RaknetListener::bind(&address)).unwrap();

        RT.block_on(server.listen());

        Ok(RaknetServer{
            server : server
        })
    }

    pub fn accept(&mut self) -> PyResult<RaknetClient> {
        let client = match RT.block_on(self.server.accept()){
            Ok(p) => p,
            Err(_) => {
                return Err(PyBaseException::new_err("accept error"));
            }
        };
        Ok(RaknetClient{
            client : client
        })
    }

    pub fn localaddr(&mut self) -> PyResult<String> {
        Ok(self.server.local_addr().unwrap().to_string())
    }

    pub fn close(&mut self) -> PyResult<()> {
        Ok(self.server.close().unwrap())
    }

    pub fn setmotd(&mut self , motd : String){
        self.server.set_full_motd(motd).unwrap();
    }
}

#[pymodule]
fn raknet_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(enablelog, m)?)?;
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    m.add_class::<RaknetClient>()?;
    m.add_class::<RaknetServer>()?;
    Ok(())
}