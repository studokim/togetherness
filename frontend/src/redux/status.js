import { createSlice } from '@reduxjs/toolkit'
import axios from 'axios'
import { getCookie, setCookie } from "../helpers/cookies.js"


export const status = createSlice({
  name: 'counter',
  initialState: {
    id: '111111',
    timer: null,
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
    gold: null,
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
    },
    setTimer: (state, action) => {
      console.log(action.payload)
      state.timer = action.payload.timer;
    },
    setId: (state, action) => {
      console.log(action.payload)
      state.id = action.payload.id;
    },
    checkName: (name) => {
      axios.get(`http://127.0.0.1:8080/api/player/${name}`)

        .then(res => {
          const persons = res.data;
          console.log(persons);
        })
        .catch(error => {
          console.log(error);
        })

    },
    createPerson: (state, action) => {
      const id = new Date().getTime();
      axios.post(`http://127.0.0.1:8080/api/player`,
        {
          name: state.name,
          avatar_id: state.selectedAvatar,
          faction_id: state.fraction,
          id: id
        })
        .then(res => {
          console.log(res.data);
          if (res.data.ok) {
            action.payload.callback(res.data.timer, id);
            setCookie("id", id)
          }
        })
    },

    getPerson: (state, action) => {
      console.log(action.payload.id)
      axios.get(`http://127.0.0.1:8080/api/player/${action.payload.id}`)

        .then(res => {
          const persons = res.data;
          console.log(persons);
        })
        .catch(error => {
          console.log(error);
        })

    },

  },
})

export const { setName, setFraction, setAvatar, createPerson, setTimer, setId, getPerson } = status.actions

export default status.reducer