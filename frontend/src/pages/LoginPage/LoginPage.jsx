import React, { useEffect } from 'react'
import "./LoginPage.scss"
import CustomInput from '../../UI/CustomInput/CustomInput'
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useNavigate } from 'react-router-dom';
import { useSelector, useDispatch } from 'react-redux'
import { setName, setAvatar } from '../../redux/status'
// import { getCookie } from "../../helpers/cookies.js"

export function LoginPage() {
    const navigator = useNavigate();
    const name = useSelector((state) => state.status.name);
    const avatars = useSelector(state => state.status.avatars);
    const selectedAvatar = useSelector(state => Number(state.status.selectedAvatar));
    const dispatch = useDispatch();



    function decrementAvatar() {
        if (selectedAvatar === 0) dispatch(setAvatar(avatars.length - 1));
        else dispatch(setAvatar(selectedAvatar - 1));
    }

    function incrementAvatar() {
        if (selectedAvatar === avatars.length - 1) dispatch(setAvatar(0))
        else dispatch(setAvatar(selectedAvatar + 1))
    }
    return (
        <div className='LoginPage'>
            <label className='nameInputLabel'>Имя вашего персонажа:</label>
            <CustomInput value={name} onChange={(e) => { dispatch(setName(e.target.value)) }} />
            <span className='nameInputLabel'>Ваш аватар:</span>
            <div className='avatarSelector'>
                <button className='arrow left' onClick={() => decrementAvatar()}></button>
                <img src={avatars[selectedAvatar]} alt='none'></img>
                <button className='arrow' onClick={() => incrementAvatar()}></button>
            </div>
            <CustomButton onClick={() => {
                if (name !== "") {
                    console.log("selectedAvatar ", selectedAvatar)
                    if (selectedAvatar === null) {dispatch(setAvatar(0)); console.log("selectedAvatar setted to 0 ")}
                    navigator("fraction");
                }
            }} disabled={name === ""}>Далее</CustomButton>
        </div >
    )
}
