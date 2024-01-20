import { createSlice } from '@reduxjs/toolkit'
import axios from 'axios'
import { setCookie } from "../helpers/cookies.js"

const ADDR = process.env.REACT_APP_ADDR ? process.env.REACT_APP_ADDR : 'http://127.0.0.1:8080/api';
if (!process.env.REACT_APP_ADDR) console.warn("Не обнаружена переменная океружения");

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
    fractionsImgs: [
      'images/fractions/cat.jpg',
      'images/fractions/crash.jpeg',
      'images/fractions/dog.jpg',
      'images/fractions/nothing.png',
    ],
    selectedAvatar: 0,
    gold: null,
    actions: null,
    subjectActions: null,       //действия совершенные игроком
    objectActions: null,        //действия совершенные над игроком
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
      // console.log("setTimer", action.payload)
      state.timer = action.payload;
    },
    setId: (state, action) => {
      console.log(action.payload)
      state.id = action.payload;
    },
    setGold: (state, action) => {
      state.gold = action.payload;
    },

    //_______
    setActions: (state, action) => {
      console.log(action.payload)
      state.actions = action.payload;
    },
    //_______

    setSubjectActions: (state, action) => {
      console.log(action.payload)
      state.subjectActions = action.payload;
    },
    setObjectActions: (state, action) => {
      console.log(action.payload)
      state.objectActions = action.payload;
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
            action.payload.callback(id, res.data.timer);
            setCookie("togethernessId", id)
          }
        })
    },
    //_______ При обновлении страницы запрашивает данные игрока и передаёт результат в callback в котором все сеттеры на имя, аватар, фракцию и прочее ___________//
    getPerson: (state, action) => {
      // console.log(action.payload)
      axios.get(`${ADDR}/player/${action.payload.id}`)
        .then(res => {
          const persons = res.data;
          console.log("getPerson", persons);
          action.payload.callback(res.data.player.name, res.data.player.id, res.data.player.avatar_id, res.data.player.faction_id)
        })
        .catch(error => {
          console.log(error);
        })
    },

    //_______ Взаимодействие с другим персонажем ___________//
    createAction: (state, action) => {
      axios.post(`${ADDR}/action`,
        {
          subject_id: state.id,
          object_id: action.payload.targetId,
          action_id: action.payload.actionId,
        })
        .then(res => {
          console.log(res.data);
          if (res.data.ok) {
            console.log("action success", action.payload)
          }
        })
    },
    getGold: (state, action) => {
      axios.get(`${ADDR}/gold/${state.id}`)
        .then(res => {
          const gold = res.data.gold;
          action.payload.callback(gold);
        })
        .catch(error => {
          console.log(error);
        })
    },
    getTimer: (state, action) => {
      axios.get(`${ADDR}/timer`)
        .then(res => {
          const timer = res.data.seconds;
          action.payload.callback(timer);
        })
        .catch(error => {
          console.log(error);
        })
    },

    //_______
    getActions: (state, action) => {
      axios.get(`${ADDR}/status/${state.id}`)
        .then(res => {
          const actions = res.data.status;
          console.log(actions)
          action.payload.callback(actions);
        })
        .catch(error => {
          console.log(error);
        })
    },
    //_______

    getObjectActions: (state, action) => {
      axios.get(`${ADDR}/action?object_id=${state.id}`)
        .then(res => {
          const actions = res.data.count;
          console.log("object ", actions)
          action.payload.callback(actions);
        })
        .catch(error => {
          console.log(error);
        })
    },
    getSubjectActions: (state, action) => {
      axios.get(`${ADDR}/action?subject_id=${state.id}`)
        .then(res => {
          const actions = res.data.count;
          console.log("subject ", actions);
          action.payload.callback(actions);
        })
        .catch(error => {
          console.log(error);
        })
    },
    actionEnabled: (state, action) => {
      axios.get(`${ADDR}/action?subject_id=${state.id}&object_id=${action.payload.targetId}`)
        .then(res => {
          const possible = res.data.possible;
          console.log(possible);
          console.log(res.data.error);
          action.payload.callback(possible);
        })
        .catch(error => {
          console.log(error);
        })
    },
  },
})

export const {
  setName, setFraction, setAvatar, createPerson, setTimer, setId, getPerson, createAction, getGold, setGold, getTimer, getActions, setActions,
  actionEnabled, getSubjectActions, getObjectActions, setSubjectActions, setObjectActions,
} = status.actions

export default status.reducer