import { useCallback, useEffect, useState } from "react";
import axios from 'axios'


export default function useGetUrl() {
    const [contacts, setContacts] = useState([]);
    const [error, setError] = useState(null);
    
    return useCallback(() => {
        axios("http://127.0.0.1:7878/api")
          .then((response: { data: any; }) => {
            setContacts(response.data);
            setError(null);
          })
          .catch(setError);
      }, [setContacts, setError]);
}