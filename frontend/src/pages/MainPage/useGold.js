import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { getGold, setGold } from "../../redux/status";

export const useGold = () => {

    const gold = useSelector((state) => state.status.gold);
    const id = useSelector((state) => state.status.id);
    const dispatch = useDispatch();
    // const [gold, setGold] = useState(null);

    useEffect(() => {

        console.log("GOLD");
        const interval = setInterval(() => {
            dispatch(getGold({ id, callback: (val) => { dispatch(setGold(val)) } }))
            console.log("gold", gold);
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    return gold
}