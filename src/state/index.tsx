import { createGlobalState } from 'react-hooks-global-state';

const { setGlobalState, useGlobalState } = createGlobalState({
    mainIndex: 0
});

export {setGlobalState, useGlobalState};