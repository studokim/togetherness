import React, { useState } from 'react';
import "./FractionPage.scss";
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useNavigate } from 'react-router-dom';
import OneFraction from './OneFraction';
import { useDispatch, useSelector } from 'react-redux';
import { createPerson, setId } from '../../redux/status'

const FractionPage = () => {
    const navigator = useNavigate();
    const fraction = useSelector((state) => state.status.fraction);
    const dispatch = useDispatch();
    // const currentFraction = useSelector((state)=>state.status.fraction)

    return (
        <div className='FractionPage'>
            <div className='header'>Выберите фракцию</div>
            <div className='fractionContainer'>
                <OneFraction fraction={1} selected={fraction === 1} url={'/images/fractions/dog.jpg'} />
                <OneFraction fraction={2} selected={fraction === 2} url={'/images/fractions/nothing.png'} />
                <OneFraction fraction={3} selected={fraction === 3} url={'/images/fractions/crash.jpeg'} />
                <OneFraction fraction={4} selected={fraction === 4} url={'/images/fractions/cat.jpg'} />
            </div>
            <CustomButton
                onClick={() => {
                    //если запускаем в ДОКЕР(есть переменная окружения), то есть проверка выбора фракции.
                    if (process.env.REACT_APP_ADDR) {
                        if (fraction !== null) navigator("/main"); dispatch(createPerson({ callback: (id) => { dispatch(setId(id)); } }))
                    }
                    //если запускаем локально или на хостинге, то просто перходим в main
                    else navigator("/main");
                }}
                disabled={fraction === null && !process.env.REACT_APP_ADDR}
            >
                Далее
            </CustomButton>
        </div>
    );
}

export default FractionPage;