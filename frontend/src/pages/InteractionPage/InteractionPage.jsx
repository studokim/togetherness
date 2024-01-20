import React, { useEffect, useState } from 'react';
import "./InteractionPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useDispatch, useSelector } from 'react-redux';
import { actionEnabled, createAction, getPerson } from '../../redux/status';

const InteractionPage = ({ targetId, close, id }) => {

    const [interractionEnebled, setInterractionEnebled] = useState(null);
    const fractionImg = useSelector(state => state.status.fractionsImgs);
    const myFractionId = useSelector(state => state.status.fraction);
    const [target, setTarget] = useState({ name: null, avatarId: null, targetId: targetId, fractionId: null });
    const dispatch = useDispatch();

    useEffect(() => {

        if (targetId !== null) {
            dispatch(getPerson({
                targetId, callback: (name, uuuuu, avatarId, fractionId) => {//при получении данных 
                    setTarget({ name: name, avatarId: avatarId, targetId: targetId, fractionId: fractionId });
                    dispatch(actionEnabled((val) => setInterractionEnebled(val)));
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

            <div>{targetId}</div>
            {/* ________________ ИМЯ ЦЕЛИ ДЛЯ ВЗАИМОДЕЙСТВИЯ ____________________ */}
            <div className='targetName'>{target.name}</div>
            {/* ________________ АВАТАР С ФРАКИЦЕЙ ____________________ */}
            <div className='imgContainer'>
                <img className='targetAvatar' src={`/images/unicorn.jpg`} />

                {target.fractionId === myFractionId
                    ?
                    <div className='fractionImg'>
                        <img src={`${fractionImg[target.fractionId]}`} />
                    </div>
                    :
                    null}
            </div>
            {/* ________________ ВЗАИМОДЕЙСТВИЯ ____________________ */}
            {interractionEnebled === null //еще не загрузились данные
                ?
                null
                :
                interractionEnebled === true //можно взаимодействовать
                    ?
                    < div className='interractions'>
                        <CustomButton onClick={() => { action(1) }}>Обнять +1/+1</CustomButton>
                        <CustomButton onClick={() => { action(2) }}>Подслушать +2/0</CustomButton>
                        <CustomButton onClick={() => { action(3) }}>Шанатажировать +3/-1</CustomButton>
                        <CustomButton onClick={() => { action(4) }}>Пустить слухи +3/0</CustomButton>
                        <CustomButton onClick={() => { action(5) }}>Преступление +4/-2</CustomButton>
                        <CustomButton onClick={() => { close() }}>Уйти</CustomButton>
                    </div>
                    : //нельзя взаимодействовать
                    <>
                        <h1>Больше ничего нельзя сделать</h1>
                        <CustomButton onClick={() => { close() }}>Уйти</CustomButton>
                    </>
            }
        </div >
    );
}

export default InteractionPage;
