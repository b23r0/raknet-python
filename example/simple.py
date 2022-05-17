from raknet_python import RaknetServer,RaknetClient

server = RaknetServer("127.0.0.1:19132")
client1 = RaknetClient("127.0.0.1:19132")

client2 = server.accept()
client1.send([0xfe , 0x01 , 0x01 , 0x01])
buf = client2.recv()
assert buf == [0xfe , 0x01 , 0x01 , 0x01]