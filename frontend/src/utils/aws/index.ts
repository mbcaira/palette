import AWS from "aws-sdk";
import { AWSConfig } from "../types/aws.types";
import {LRUCache} from 'lru-cache'


const configCache: LRUCache<string, AWSConfig> = new LRUCache({
  ttl: 1000*60*5,
  max: 500,
  size: 1
})

export const getConfig = ():AWSConfig => {
  const confKey = "conf"
  if (!configCache.has(confKey)) {
    const bucketName =  process.env.BUCKET_NAME as string
    const region = process.env.REGION as string
    const identityPool = process.env.IDENTITY_POOL as string

    if (!bucketName || !region || !identityPool) {
      throw new Error("Could not read values from env")
    }

    const conf: AWSConfig = {
      bucketName, region, identityPool
    }
    configCache.set(confKey, conf)
  }

  return configCache.get(confKey)!
}


export const setConfig = () => {
  const config = getConfig();

  AWS.config.update({
    region: config.region,
    credentials: new AWS.CognitoIdentityCredentials({
      IdentityPoolId: config.identityPool
    })
  })
}