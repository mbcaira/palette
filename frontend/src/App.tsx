import { ChakraProvider, Container, theme } from '@chakra-ui/react'
import Upload from './components/Upload'
import Navbar from './components/Navbar'
export const App = () => {
  return (
    <ChakraProvider theme={theme}>
      <Navbar />
      <Container as="main" pt="10">
        <Upload />
      </Container>
    </ChakraProvider>
  )
}
