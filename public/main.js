async function init() {
  let rustApp = null;

  try {
    rustApp = await import('../pkg');
  } catch (err) {
    console.error(err);
    return;
  }

  console.log(rustApp);

  const fileReader = new FileReader();
  const input = document.getElementById('upload');

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
    
    const encoded_result = rustApp.grayscale(base64);

    document.getElementById('new-img').setAttribute('src', encoded_result);
  };

  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();