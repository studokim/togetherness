import React, { useEffect, useState } from 'react';
import "./InteractionPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useDispatch, useSelector } from 'react-redux';
import { actionEnabled, createAction, getPerson } from '../../redux/status';

const InteractionPage = ({ targetId, close, id }) => {

    const [interractionEnebled, setInterractionEnebled] = useState(null);
    const fractionImg = useSelector(state => state.status.fractionsImgs);
    const myFractionId = useSelector(state => state.status.fraction);
    const avatars = useSelector(state => state.status.avatars);


    const [target, setTarget] = useState({ name: null, avatarId: null, targetId: targetId, fractionId: null });
    const dispatch = useDispatch();

    useEffect(() => {

        if (targetId !== null) {
            dispatch(getPerson({
                id: targetId, callback: (name, uuuuu, avatarId, fractionId) => {//при получении данных 
                    setTarget({ name: name, avatarId: avatarId, targetId: targetId, fractionId: fractionId });
                    dispatch(actionEnabled({ targetId: targetId, callback: (val) => setInterractionEnebled(val) }));
                },
                errorHandler: () => { }
            }));
        }
        console.log("targetId", targetId)
    }, [targetId])

    function action(actionId) {
        dispatch(createAction({ targetId, actionId }));
        close();
    }
    console.log("target.fractionId, ", target.fractionId);
    console.log("myFractionId, ", myFractionId);
    console.log("Number(target.fractionId) - 1, ", Number(target.fractionId) - 1);
    console.log("fractionImg[Number(target.fractionId) - 1], ", fractionImg[Number(target.fractionId) - 1]);

    return (
        <div className='InteractionPage'>

            <div>{targetId}</div>
            {/* ________________ ИМЯ ЦЕЛИ ДЛЯ ВЗАИМОДЕЙСТВИЯ ____________________ */}
            <div className='targetName'>{target.name}</div>
            {/* ________________ АВАТАР С ФРАКИЦЕЙ ____________________ */}
            <div className='imgContainer'>
                <img className='targetAvatar' src={target.avatarId !== null ? avatars[target.avatarId] : `/images/unicorn.jpg`} />

                {target.fractionId === myFractionId
                    ?
                    <div className='fractionImg'>
                        <img src={`${fractionImg[Number(target.fractionId) - 1]}`} />
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
