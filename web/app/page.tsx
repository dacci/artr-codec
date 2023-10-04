'use client';

import { useContext, useState } from 'react';
import { WasmContext } from '@/context';
import { Button, ButtonGroup, IconButton, Snackbar, Stack, TextField } from '@mui/material';
import { Close } from '@mui/icons-material';

export default function Home() {
  const { artr } = useContext(WasmContext);

  const [rawData, setRawData] = useState('');
  const [artrData, setArtrData] = useState('');
  const [message, setMessage] = useState('');
  const [snackbarOpen, setSnackbarOpen] = useState(false);

  const handleEncode = () => {
    try {
      setArtrData(artr!.encode(rawData));
    } catch (e: any) {
      setMessage(e.toString());
      setSnackbarOpen(true);
    }
  };

  const handleDecode = () => {
    try {
      setRawData(artr!.decode(artrData));
    } catch (e: any) {
      setMessage(e.toString());
      setSnackbarOpen(true);
    }
  };

  const handleClose = (e: unknown, reason?: string) => {
    if (reason !== 'clickaway') setSnackbarOpen(false);
  };

  const style = { height: '100%' };

  return (
    <>
      <Stack
        spacing={1}
        style={style}
        sx={{ p: 1 }}
      >
        <TextField
          FormHelperTextProps={{ style }}
          InputLabelProps={{ style }}
          InputProps={{ style }}
          inputProps={{ style }}
          label='本音'
          multiline
          placeholder='本音を書いてください'
          style={style}
          value={rawData}
          onChange={(e) => setRawData(e.target.value)}
        />
        <ButtonGroup variant='contained' style={{ marginLeft: 'auto', marginRight: 'auto' }}>
          <Button onClick={handleEncode}>🤫 建前化</Button>
          <Button onClick={handleDecode}>🫣 本音化</Button>
        </ButtonGroup>
        <TextField
          FormHelperTextProps={{ style }}
          InputLabelProps={{ style }}
          InputProps={{ style }}
          inputProps={{ style }}
          label='建前'
          multiline
          placeholder='建前を書いてください'
          style={style}
          value={artrData}
          onChange={(e) => setArtrData(e.target.value)}
        />
      </Stack>
      <Snackbar
        action={
          <IconButton
            aria-label='close'
            color='inherit'
            size='small'
            onClick={handleClose}
          >
            <Close fontSize='small'/>
          </IconButton>
        }
        autoHideDuration={8000}
        message={message}
        open={snackbarOpen}
        onClose={handleClose}
      />
    </>
  );
}
