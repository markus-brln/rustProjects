import { Flex, Button, Text } from '@mantine/core';
import { useCallback, useState } from 'react';
import useGetUrl from '../../hooks/useGetUrl/useGetUrl';


export default function HomePage() {
  const [responseText, setResponseText] = useState(null)
  const getUrl = useGetUrl();

  const onClick = useCallback(() => getUrl().then(data => setResponseText(data)), 
    [getUrl, setResponseText]
  );

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
    <Button onClick={onClick}>Button 1</Button>
    <Text>{JSON.stringify(responseText, null, "\t")}</Text>
  </Flex>
  );
}
