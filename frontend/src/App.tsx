import { ChangeEvent, useState } from "react";
import {
  ChakraProvider,
  Box,
  Grid,
  theme,
  Button,
  Input,
} from "@chakra-ui/react";
import { ColorModeSwitcher } from "./ColorModeSwitcher";
import { uploadFile } from "./utils/uploadFile";
export const App = () => {
  const [file, setFile] = useState<File | null>(null);

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    } else {
      setFile(null);
    }
  };

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid minH="100vh" p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <Input type="file" onChange={handleFileChange} />
          <Button
            onClick={(_) => {
              console.log(file);
              if (file != null) {
                uploadFile(file);
              }
            }}
          >
            Submit
          </Button>
        </Grid>
      </Box>
    </ChakraProvider>
  );
};
