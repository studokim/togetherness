import React, { useState } from 'react';
import "./FractionPage.scss";
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useNavigate } from 'react-router-dom';
import OneFraction from './OneFraction';
import { useSelector } from 'react-redux';

const FractionPage = () => {
    const navigator = useNavigate();
    const fraction = useSelector((state) => state.status.fraction);

    return (
        <div className='FractionPage'>
            <div className='header'>Выберите фракцию</div>
            <div className='fractionContainer'>
                <OneFraction fraction={1} selected={fraction === 1} url={'/images/dog.jpg'} />
                <OneFraction fraction={2} selected={fraction === 2} url={'/images/nothing.png'} />
                <OneFraction fraction={3} selected={fraction === 3} url={'/images/crash.jpeg'} />
                <OneFraction fraction={4} selected={fraction === 4} url={'/images/cat.jpg'} />
            </div>
            <CustomButton onClick={() => { if (fraction !== null) navigator("/main") }}>Далее</CustomButton>
        </div>
    );
}

export default FractionPage;