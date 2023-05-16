clc
clear

fileName = 'imbd_sentiment.csv';

data = readtable(fileName,'TextType','string');
textData = data.review;
docs = tokenizedDocument(textData);
docs = addPartOfSpeechDetails(docs);
docs = erasePunctuation(docs);
docs = removeStopWords(docs);
docs = removeShortWords(docs,2);
docs = removeLongWords(docs,15);
docs = removeEmptyDocuments(docs);
docs = addLemmaDetails(docs);

compoundScores = vaderSentimentScores(docs);

pos = compoundScores > 0;
strPositive = docs(pos);

neut = compoundScores == 0;
strNeutral = docs(neut);

neg = compoundScores < 0;
strNegative = docs(neg);

figure
subplot(1,3,1)
wordcloud(strPositive);
title("Positive Sentiment")

subplot(1,3,2)
wordcloud(strNegative);
title("Negative Sentiment")

subplot(1,3,3)
wordcloud(strNeutral);
title("Neutral Sentiment")