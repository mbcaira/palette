import AWS from 'aws-sdk'
import type { ManagedUpload } from 'aws-sdk/lib/s3/managed_upload'
import { getConfig, setConfig } from '../aws'
import { Colour } from '../types/palette.types'

export const uploadFile = async (file: File): Promise<Colour[]> => {
  setConfig()
  const key = encodeURIComponent(file.name)
  const upload = new AWS.S3.ManagedUpload({
    params: {
      Bucket: getConfig().bucketName,
      Key: key,
      Body: file,
    },
  })

  const colours: Colour[] = []

  // On upload success, the UI needs to call the server with the image's path on S3 to be processed
  // Then, once a response from the server is received, send back the colour values to the frontend
  upload
    .promise()
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    .then((val: ManagedUpload.SendData) => {
      colours.push({ r: 255, g: 0, b: 0, a: 1 })
    })
    .catch((err) => {
      console.error(err)
    })

  return colours
}
