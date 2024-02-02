import { ChakraProvider, theme } from '@chakra-ui/react';
import Upload from './components/Upload';
export const App = () => {
  return (
    <ChakraProvider theme={theme}>
      <Upload />
    </ChakraProvider>
  );
};
