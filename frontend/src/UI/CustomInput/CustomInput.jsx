import React from 'react'
import "./CustomInput.scss"

export default function CustomInput({ ...props }) {


    return (
        <input {...props} className='CustomInput' />
    )
}
