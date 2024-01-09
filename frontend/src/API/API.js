import axios from 'axios'

export function checkName(name) {
    axios.get(`http://127.0.0.1:8080/api/player/${name}`)

        .then(res => {
            const persons = res.data;
            console.log(persons);
        })
        .catch(error => {
            console.log(error);
        })

}

export function createPerson(name, avatar_id, faction_id, id, callback) {
    axios.post(`http://127.0.0.1:8080/api/player`, { name, avatar_id, faction_id, id })
        .then(res => {
            console.log(res.data);
            if (res.data.ok) callback(res.data);
        })
}

export function action(name, avatar_id, faction_id, id, callback) {
    axios.post(`http://127.0.0.1:8080/api/player`, { name, avatar_id, faction_id, id })
        .then(res => {
            console.log(res.data);
            if (res.data.ok) callback(res.data);
        })
}