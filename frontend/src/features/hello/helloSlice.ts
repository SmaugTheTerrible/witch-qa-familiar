import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { fetchHelloFromAPI } from "./helloAPI";

export interface HelloState {
  response: string;
  error: string;
  loading: boolean;
}

const initialState: HelloState = { response: "", loading: false, error: "" };

export const fetchHello = createAsyncThunk(
  "hello/fetchHello",
  async (name: string, thunkAPI) => {
    try {
      const res = await fetchHelloFromAPI(name);
      return res;
    } catch (e) {
      return thunkAPI.rejectWithValue(String(e));
    }
  }
);

export const helloSlice = createSlice({
  name: "hello",
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchHello.pending, (state) => {
        state.loading = true;
      })
      .addCase(fetchHello.fulfilled, (state, action) => {
        state.loading = false;
        state.response = action.payload;
      })
      .addCase(fetchHello.rejected, (state) => {
        state.loading = false;
        // state.error = action.payload;
      });
  },
});

export default helloSlice.reducer;
