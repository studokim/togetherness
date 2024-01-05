import React from 'react';
import { useDispatch } from 'react-redux';
import { setFraction } from '../../redux/status'

const OneFraction = ({ fraction, selected, url }) => {
    const dispatch = useDispatch();

    return (
        <div className={'oneFraction' + (selected === true ? ' selected' : "")} onClick={() => { dispatch(setFraction(fraction)) }}>
            <img src={url} />
        </div>
    );
}

export default OneFraction;