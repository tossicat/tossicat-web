const tc = import("./node_modules/@dialektike/tossicat/tossicat.js");
  tc.then(tc => {

    const fix_button = document.getElementById("fixbutton");
    fix_button.addEventListener("click", event => {
        try {
            wordinput = document.getElementById("wordinput").value;
            tossinput = document.getElementById("tossiinput").value;
            result = tc.fix(wordinput,tossinput);
            document.getElementById("fixresult").innerHTML = result;
        } catch (err) {
            document.getElementById("fixresult").innerHTML = err;
        }
      });

    const sentence_button = document.getElementById("sentencebutton");
    sentence_button.addEventListener("click", event => {
          try {
              sentenceinput = document.getElementById("sentenceinput").value;
              console.log(sentenceinput);
              result = tc.fix_sentence(sentenceinput);
              document.getElementById("sentenceresult").innerHTML = result;
          } catch (err) {
              document.getElementById("sentenceresult").innerHTML = err;
          }
        });
  })
  .catch(console.error);