import { Flex, Button, Text, Image } from '@mantine/core';
import { useCallback, useState } from 'react';
import useGetUrl from '../../hooks/useGetUrl/useGetUrl';
import { FaBrain } from "react-icons/fa";


export default function HomePage() {
  const [responseText, setResponseText] = useState(null);
  const [responseImg, setResponseImg] = useState(undefined);

  const getUrl = useGetUrl();

  const onClickSetJsonText = useCallback((url: string) => getUrl(url).then(data => setResponseText(data)), 
    [getUrl, setResponseText]
  );

  const onClickSetImg = useCallback((url: string) => getUrl(url).then(data => setResponseImg(data.data)), 
  [getUrl, setResponseImg]
);

  const reset = useCallback(() => {
    setResponseText(null);
    setResponseImg(undefined);
  }, [setResponseImg, setResponseText])

  return (
    <Flex
    mih={50}
    bg="rgba(0, 0, 0, .1)"
    gap="md"
    justify="flex-start"
    align="flex-start"
    direction="column"
    wrap="wrap"
  >
    <Button onClick={() => reset()}>Reset</Button>
    <Button onClick={() => onClickSetJsonText("get")}>Get JSON</Button>
    <Button onClick={() => onClickSetImg("get-image")}>Get Image base64</Button>
    <Text>{JSON.stringify(responseText, null, "\t")}</Text>
    <Image
      width={500}
      height={300}
      src={responseImg}
      withPlaceholder
      placeholder={<FaBrain size={50}/>}
      >

      </Image>
  </Flex>
  );
}
