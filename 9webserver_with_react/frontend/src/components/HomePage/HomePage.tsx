import { Flex, Button } from '@mantine/core';
import useGetUrl from '../../hooks/useGetUrl/useGetUrl';


export default function HomePage() {
  const getUrl = useGetUrl();

  return (
    <Flex
    mih={50}
    bg="rgba(0, 0, 0, .1)"
    gap="md"
    justify="flex-start"
    align="flex-start"
    direction="row"
    wrap="wrap"
  >
    <Button onClick={() => getUrl()}>Button 1</Button>
  </Flex>
  );
}
