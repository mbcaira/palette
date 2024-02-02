import { Grid, Input, Button, Box } from '@chakra-ui/react'
import { ChangeEvent, useState } from 'react'
import { uploadFile } from '../../utils/uploadFile'
import { ColorModeSwitcher } from '../../ColorModeSwitcher'

export const Upload = () => {
  const [file, setFile] = useState<File | null>(null)
  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0])
    } else {
      setFile(null)
    }
  }

  return (
    <Box textAlign="center" fontSize="xl">
      <Grid minH="100vh" p={3}>
        <ColorModeSwitcher justifySelf="flex-end" />
        <Input type="file" onChange={handleFileChange} />
        <Button
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          onClick={(_) => {
            console.log(file)
            if (file != null) {
              uploadFile(file)
            }
          }}
        >
          Submit
        </Button>
      </Grid>
    </Box>
  )
}

export default Upload
