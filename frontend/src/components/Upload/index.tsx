import { Grid, Input, Button, VStack, GridItem } from '@chakra-ui/react'
import { ChangeEvent, useEffect, useState } from 'react'
import { uploadFile } from '../../utils/uploadFile'
import { Colour } from '../../utils/types/palette.types'

export const Upload = () => {
  const [file, setFile] = useState<File | null>(null)
  const [colours, setColours] = useState<Colour[]>([])
  const [uploadClicked, setUploadClicked] = useState<boolean>(false)

  // Update file name if the target is valid
  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0])
    } else {
      setFile(null)
    }
  }

  // Whenever upload is clicked, begin processing logic
  // Bucket -> Server -> back to frontend
  useEffect(() => {
    const updateColours = async (file: File) => {
      const colours = await uploadFile(file)
      setColours(colours)
    }
    if (uploadClicked) {
      updateColours(file)
      setUploadClicked(false)
    }
  }, [uploadClicked])

  return (
    <VStack>
      <Input type="file" onChange={handleFileChange} />
      <Button minW="100%" onClick={() => setUploadClicked(true)}>
        Submit
      </Button>
      <Grid templateColumns="repeat(5, 1fr)" gap={6}>
        {colours.map((colour: Colour) => {
          const { r, g, b, a } = colour
          return <GridItem bg={`rgba(${r},${g}, ${b}, ${a})`}>Hello</GridItem>
        })}
      </Grid>
    </VStack>
  )
}

export default Upload
