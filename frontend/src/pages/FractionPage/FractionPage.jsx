import React, { useEffect } from 'react';
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

    // useEffect(() => {
    //     const id = getCookie("togethernessId");
    //     console.log(id);
    //     //Если в куках есть id
    //     if (id !== undefined) {
    //       dispatch(getPerson({
    //         id, callback: (name, id, avatarId, fractionId, timer) => {
    //           dispatch(setName(name));
    //           dispatch(setId(id));
    //           dispatch(setFraction(fractionId));
    //           dispatch(setAvatar(avatarId));
    //           navigator("/main");
    //         },
    //         errorHandler: (error) => {
    //           console.log("Пользователь не зарегистрирован. ", error);
    //           navigator("/");
    //         }
    //       }));
    //       console.log("Обновление");
    //     }
    //     else {
    //       navigator("/");
    //     }
    //   }, [])


    return (
        <div className='FractionPage'>
            {/* <div className="FractionPageInner"> */}
            <div className='header'>Выберите фракцию</div>
            <div className='fractionContainer'>
                <OneFraction fraction={1} selected={fraction === 1} url={'/images/fractions/иконка1.jpg'} />
                <OneFraction fraction={2} selected={fraction === 2} url={'/images/fractions/иконка2.jpg'} />
                <OneFraction fraction={3} selected={fraction === 3} url={'/images/fractions/иконка3.jpg'} />
                <OneFraction fraction={4} selected={fraction === 4} url={'/images/fractions/иконка4.jpg'} />
            </div>
            <CustomButton
                someClass="red"
                onClick={() => {
                    //если запускаем в ДОКЕР(есть переменная окружения), то есть проверка выбора фракции.
                    if (process.env.REACT_APP_ADDR) {
                        if (fraction !== null) dispatch(createPerson({ callback: (id) => { dispatch(setId(id)); navigator("/main"); } }))
                    }
                    //если запускаем локально или на хостинге, то просто перходим в main
                    else navigator("/main");
                }}
                disabled={fraction === null && !process.env.REACT_APP_ADDR}
            >
                Далее
            </CustomButton>
            {/* </div> */}
        </div>
    );
}

export default FractionPage;