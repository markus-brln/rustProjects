import axios from 'axios'
import { useCallback } from 'react';


export default function useGetUrl() {
    const baseUrl = "http://localhost:8000";

    return useCallback((url: string) => axios.get(baseUrl + "/" + url)
          .then((response: { data: any; }) => {
            return response.data;
          })
          .catch(error => error.message), [])
}