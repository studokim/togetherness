import { createSlice } from '@reduxjs/toolkit'
import axios from 'axios'
import { getCookie, setCookie } from "../helpers/cookies.js"

const ADDR = 'http://127.0.0.1:8080/api';

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
      axios.get(`${ADDR}/player/${name}`)

        .then(res => {
          const persons = res.data;
          console.log(persons);
        })
        .catch(error => {
          console.log(error);
        })

    },
    createPerson: (state, action) => {
      const id = new Date().getTime().toString();
      axios.post(`${ADDR}/player`,
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
            setCookie("togethernessId", id)
          }
        })
    },
    //_______ При обновлении страницы запрашивает данные игрока и передаёт результат в callback в котором все сеттеры на имя, аватар, фракцию и прочее ___________//
    getPerson: (state, action) => {
      console.log(action.payload)
      axios.get(`${ADDR}/player/${action.payload.id}`)
        .then(res => {
          const persons = res.data;
          console.log(persons);
          action.payload.callback(res.data.player.name, res.data.player.id, res.data.player.avatar_id, res.data.player.faction_id)
        })
        .catch(error => {
          console.log(error);
        })

    },

  },
})

export const { setName, setFraction, setAvatar, createPerson, setTimer, setId, getPerson } = status.actions

export default status.reducer