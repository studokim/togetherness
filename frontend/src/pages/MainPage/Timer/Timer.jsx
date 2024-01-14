import React, { useState, useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux';
import { getTimer, setTimer } from '../../../redux/status';
import "./Timer.scss"

export function Timer() {

    const timer = useSelector((state) => state.status.timer);
    const dispatch = useDispatch();
    const [timerString, setTimerString] = useState({ minutes: "ХХ", seconds: "ХХ" })

    useEffect(() => {

        const interval = setInterval((timer) => {
            dispatch(getTimer({ callback: (timer) => dispatch(setTimer(timer)) }));
        }, 1000);

        if (timer !== null) {
            let tempTimer = timer;
            const minutes = Math.floor(tempTimer / 60) < 10 ? "0" + Math.floor(tempTimer / 60) : Math.floor(tempTimer / 60);
            const seconds = tempTimer % 60 < 10 ? "0" + tempTimer % 60 : tempTimer % 60;
            setTimerString({ minutes: minutes, seconds: seconds });
        }

        return () => clearInterval(interval);

    }, [timer]);

    return (
        <div className='Timer'>
            {timer !== null ?
                <span>{timerString.minutes + ":" + timerString.seconds}</span>
                :
                <span>XX:XX</span>
            }
        </div>
    )
}
