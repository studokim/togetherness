import React, { useEffect, useState } from 'react';
import "./InteractionPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useDispatch } from 'react-redux';
import { createAction, getPerson } from '../../redux/status';

const InteractionPage = ({ targetId, close, id }) => {

    const [target, setTarget] = useState({ name: null, avatarId: null, targetId: targetId })
    const dispatch = useDispatch();
    useEffect(() => {

        if (targetId !== null) {
            dispatch(getPerson({
                targetId, callback: (name, uuuuu, avatarId, fractionId) => {//при получении данных 
                    setTarget({ name: name, avatarId: avatarId, targetId: targetId })
                }
            }));
        }
        console.log("targetId", targetId)
    }, [targetId])

    function action(actionId) {
        dispatch(createAction({ targetId, actionId }));
        close();
    }

    return (
        <div className='InteractionPage'>
            <div className='targetName'>{target.name}</div>
            <img className='targetAvatar' src={`/images/unicorn.jpg`} />
            <div className='interractions'>
                <CustomButton onClick={() => { action(1) }}>Обнять +1/+1</CustomButton>
                <CustomButton onClick={() => { action(2) }}>Подслушать +2/0</CustomButton>
                <CustomButton onClick={() => { action(3) }}>Шанатажировать +3/-1</CustomButton>
                <CustomButton onClick={() => { action(4) }}>Пустить слухи +3/0</CustomButton>
                <CustomButton onClick={() => { action(5) }}>Преступление +4/-2</CustomButton>
                <CustomButton onClick={() => { console.log("target", target, "subgect", id) }}>STATUS</CustomButton>
                <CustomButton onClick={() => { close() }}>Уйти</CustomButton>
            </div>
        </div>
    );
}

export default InteractionPage;
