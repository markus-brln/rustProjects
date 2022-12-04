import axios from 'axios'
import { useCallback } from 'react';


export default function useGetUrl() {
    
    return useCallback(() => axios.get("http://127.0.0.1:8000/get")
          .then((response: { data: any; }) => {
            return response.data;
          })
          .catch(error => error.message), [])
}