import AWS from 'aws-sdk'
import type { AWSConfig } from '../types/aws.types'
import { LRUCache } from 'lru-cache'

const configCache: LRUCache<string, AWSConfig> = new LRUCache({
  ttl: 1000 * 60 * 5,
  max: 500,
  size: 1,
})

/**
 * Reads and loads AWS configuration details
 * @returns AWS configuration, loaded from env variables
 */
export const getConfig = (): AWSConfig => {
  const confKey = 'conf'
  if (!configCache.has(confKey)) {
    const bucketName = process.env.REACT_APP_BUCKET_NAME as string
    const region = process.env.REACT_APP_REGION as string
    const identityPool = process.env.REACT_APP_IDENTITY_POOL as string
    console.log(process.env)
    if (!bucketName || !region || !identityPool) {
      throw new Error('Could not read values from env')
    }

    const conf: AWSConfig = {
      bucketName,
      region,
      identityPool,
    }
    configCache.set(confKey, conf)
  }

  return configCache.get(confKey)
}

/**
 * Sets the AWS config
 */
export const setConfig = (): void => {
  const config: AWSConfig = getConfig()
  AWS.config.update({
    region: config.region,
    credentials: new AWS.CognitoIdentityCredentials({
      IdentityPoolId: config.identityPool,
    }),
  })
}
