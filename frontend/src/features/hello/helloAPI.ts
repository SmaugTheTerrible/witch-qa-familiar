import axios from "axios";

export const fetchHelloFromAPI = async (username: string) => {
  const { data } = await axios.post<{ result: string }>("/api/hello", {
    username,
  });
  return data.result;
};
