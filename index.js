function falert() {
    console.log('123');
    alert("yo");
}

async function saveFile() {
    let formData = new FormData();
    formData.append("file", fileupload.files[0]);
    await fetch('http://127.0.0.1:7878/', {method: "POST", body: formData});
    alert('File has been uploaded');
}