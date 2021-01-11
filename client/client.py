import pprint
import sys

import grpc
import upload_image_pb2

import asyncio
import aiohttp
import binascii


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(go(loop))
    loop.close()


async def fetch(session):
    content_type = sys.argv[1]
    name = sys.argv[2]
    image = b""
    with open("image.png", "rb") as f:
        image = f.read()


    # リクエストを作成する
    upload_image = upload_image_pb2.UploadImage(content_type=content_type, name=name, image=image)
    req = upload_image_pb2.UploadImages(images=[upload_image])

    #Serialize
    sendDataStr = req.SerializeToString()
    print("serialized string:", sendDataStr)


    async with session.post('http://127.0.0.1:8088/proto', data=sendDataStr,
                            headers={"content-type": "application/protobuf"}) as resp:
        print(resp.status)
        print(type(image))
        data = await resp.read()
        receiveObj = upload_image_pb2.UploadImageResponse()
        receiveObj.ParseFromString(data)
        print(receiveObj)

async def go(loop):
    async with aiohttp.ClientSession(loop=loop) as session:
        await fetch(session)



if __name__ == '__main__':
    main()
