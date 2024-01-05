import "./CustomButton.scss"

const CustomButton = ({children, ...props}) => {
    return (
        <button className='CustomButton' {...props}>
            {children}
        </button>
    );
}

export default CustomButton;
