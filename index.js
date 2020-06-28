
import './static/style.scss';
let wasm;

  import("./pkg").then(module => {
    console.log("module", module)
    module.run_app();
    wasm = module
    initialize();
  });
  
  
const constraints = {
    video: {
        facingMode: "environment",
    },
};
var video;
var scale = 0.25;

var decodeQr = function (byteArray) {
    try {
      
      const output = wasm.decode_qr(byteArray);
      const usedOutput = output.includes("[Error]") ? null : output;
      console.log("output", usedOutput)
      if(usedOutput) {
        let htmlElement = document.getElementById("video");
        htmlElement.remove();
        let result = document.getElementById("result");
        var a = document.createElement('a');
        var linkText = document.createTextNode(usedOutput);
        a.appendChild(linkText);

        a.title = usedOutput;
        a.href = usedOutput;
        result.appendChild(a);                             

      }


    } catch (err) {
      console.log("err", err)
    }
};

var captureImage = function () {
    var canvas = document.createElement("canvas");
    canvas.width = video.videoWidth * scale;
    canvas.height = video.videoHeight * scale;
    canvas
        .getContext("2d")
        .drawImage(video, 0, 0, canvas.width, canvas.height);

    // var img = document.createElement("img");
    // img.src = canvas.toDataURL();
    window.myCanvas = canvas;
    // $output.appendChild(img);

    // const clampedByteArray = myCanvas.getContext("2d").getImageData(0, 0, 640, 480).data;

    canvas.toBlob(blob => {
        const reader = new FileReader();

        reader.addEventListener("loadend", () => {
            const arrayBuffer = reader.result;
            window.ab = arrayBuffer;

            decodeQr(new Uint8Array(arrayBuffer));
        });
        reader.readAsArrayBuffer(blob);
    });
};

var initialize = function () {
    video = document.getElementById("video");

    navigator.mediaDevices.getUserMedia(constraints).then(stream => {
        video.srcObject = stream;
    });

    setInterval(captureImage, 300);
};
