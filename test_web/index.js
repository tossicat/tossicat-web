const js = import("./node_modules/@dialektike/tossicat/tossicat.js");
    js.then(js => {
      temp_1 = js.fix("철수","가");
      temp_2 = js.fix_sentence("{철수, 은} {영희, 처럼} {밥,  를} 먹습니다.");
      temp = temp_1 + temp_2;
      alert(temp);
    });