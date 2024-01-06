import React from 'react';
import "./InteractionPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton';

const InteractionPage = () => {
    return (
        <div className='InteractionPage'>
            <div className='targetName'>targetName</div>
            <img className='targetAvatar' src={`/images/unicorn.jpg`} />
            <div className='interractions'>
                <CustomButton >Обнять +1/+1</CustomButton>
                <CustomButton >Подслушать +2/0</CustomButton>
                <CustomButton >Шанатажировать +3/-1</CustomButton>
                <CustomButton >Пустить слухи +3/0</CustomButton>
                <CustomButton >Преступление +4/-2</CustomButton>
            </div>
        </div>
    );
}

export default InteractionPage;
