import { createSlice } from '@reduxjs/toolkit'

export const status = createSlice({
  name: 'counter',
  initialState: {
    name: "",
    fraction: null,
    avatars: [
      'images/avatars/avatar-svgrepo-com (1).svg',
      'images/avatars/avatar-svgrepo-com (2).svg',
      'images/avatars/avatar-svgrepo-com (3).svg',
      'images/avatars/avatar-svgrepo-com (5).svg',
      'images/avatars/avatar-svgrepo-com (6).svg',
      'images/avatars/avatar-svgrepo-com.svg',
    ],
    selectedAvatar: 0,
  },
  reducers: {
    setName: (state, action) => {
      console.log(action)
      state.name = action.payload
    },
    setFraction: (state, action) => {
      console.log(action.payload)
      state.fraction = action.payload;
    },
    setAvatar: (state, action) => {
      console.log(action.payload)
      state.selectedAvatar = action.payload;
    }
  },
})

export const { setName, setFraction, setAvatar } = status.actions

export default status.reducer