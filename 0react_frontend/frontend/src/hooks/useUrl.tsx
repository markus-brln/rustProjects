import axios from 'axios'
import { useCallback } from 'react';


export function useGetUrl() {
    const baseUrl = "http://localhost:8000";

    return useCallback((url: string) => axios.get(baseUrl + "/" + url)
          .then((response: { data: any; }) => {
            return response.data;
          })
          .catch(error => error.message), [])
}

export function usePostUrl() {
    const baseUrl = "http://localhost:8000";

    return useCallback((url: string, payload: object) => axios.post(baseUrl + "/" + url, payload)
          .then((response: { data: any; }) => {
            return response.data;
          })
          .catch(error => error.message), [])
}