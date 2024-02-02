import AWS from 'aws-sdk'
import type { ManagedUpload } from 'aws-sdk/lib/s3/managed_upload'
import { getConfig, setConfig } from '../aws'

export const uploadFile = async (
  file: File,
): Promise<ManagedUpload.SendData | boolean> => {
  setConfig()
  const key = encodeURIComponent(file.name)
  const upload = new AWS.S3.ManagedUpload({
    params: {
      Bucket: getConfig().bucketName,
      Key: key,
      Body: file,
    },
  })

  // On upload success, the UI needs to call the server with the image's path on S3 to be processed
  let success: ManagedUpload.SendData | boolean = false
  upload
    .promise()
    .then((val: ManagedUpload.SendData) => {
      success = val
    })
    .catch((err) => {
      console.log(err)
    })

  return success
}
