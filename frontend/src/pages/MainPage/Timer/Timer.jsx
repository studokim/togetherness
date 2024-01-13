import React, { useState, useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux';
import { getTimer, setTimer } from '../../../redux/status';
import "./Timer.scss"

export function Timer() {

    const timer = useSelector((state) => state.status.timer);
    const dispatch = useDispatch();
    const [timerString, setTimerString] = useState({ minutes: "60", seconds: "00" })


    useEffect(() => {

        const interval = setInterval(() => {
            // if (timer !== null) {
            console.log(timer)

            dispatch(getTimer({ callback: (timer) => setTimer(timer) }));
            if (timer !== null) {
                let tempTimer = timer;
                // console.log(timer)
                // dispatch(setTimer(tempTimer - 1));
                const minutes = Math.floor(tempTimer / 60);
                const seconds = tempTimer % 60 < 10 ? "0" + tempTimer % 60 : tempTimer % 60;
                setTimerString({ minutes: minutes, seconds: seconds });
            }
            // else setTimerString({ minutes: minutes, seconds: seconds });

        }, 1000);

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
