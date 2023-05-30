Bu proje, SVM (Support Vector Machine) algoritmasını kullanarak spam e-postalarını filtrelemek için bir uygulama geliştirmeyi amaçlamaktadır. Google Colab ortamında Python programlama dili kullanılarak oluşturulan bu uygulama, TfidfVectorizer ile metin vektörleştirme ve SVC (Support Vector Classification) ile SVM modeli eğitimi sağlamaktadır. Eğitim veri kümesi üzerindeki modelin performansı, test veri kümesiyle değerlendirilerek sınıflandırma raporu elde edilmektedir. Ayrıca, kullanıcıya gerçek zamanlı bir demo sunularak, kullanıcının girdiği metni SVM modeliyle değerlendirerek spam veya normal e-posta olarak sınıflandırmaktadır.
**1**.	İlk olarak, gerekli kütüphaneleri import ediyoruz: pandas, train_test_split, TfidfVectorizer, SVC ve classification_report. Bu kütüphaneler, veri işleme, veri kümesi bölme, metin vektörleştirme, SVM modeli ve sınıflandırma raporu gibi işlemler için kullanılır.

**pandas**: Veri işleme ve manipülasyonu için kullanılan bir Python kütüphanesidir. Bu projede, veri setinin yüklenmesi ve kolonlara erişim için pandas kullanılıyor.
**sklearn.model_selection.train_test_split**: Veri kümesini eğitim ve test alt kümelerine bölmek için kullanılan bir fonksiyondur. Bu sayede, modelin eğitim verileri üzerinde eğitilmesi ve performansının test verileriyle değerlendirilmesi sağlanır.
**sklearn.feature_extraction.text.TfidfVectorizer**: Metin verilerini sayısal vektörlere dönüştürmek için kullanılan bir sınıftır. TfidfVectorizer, TF-IDF (Term Frequency-Inverse Document Frequency) yöntemini kullanarak metinlerin vektörel temsilini oluşturur. Bu projede, e-posta metinlerinin vektörleştirilmesi için kullanılır.
**sklearn.svm.SVC**: Support Vector Classification (SVC) algoritmasını uygulayan bir sınıftır. SVM algoritmasını kullanarak sınıflandırma problemlerini çözer. Bu projede, eğitim verileri üzerindeki SVM modelini oluşturmak ve eğitmek için kullanılır.
**sklearn.metrics.classification_report**: Sınıflandırma modelinin performansını değerlendirmek için kullanılan bir fonksiyondur. Precision, recall, f1-score ve support gibi metrikleri sağlar. Bu projede, modelin sınıflandırma performansını raporlamak için kullanılır.

![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/d5531ca4-efe5-459f-b18e-195e30a0e7ef)

**2**.	**spam_ham_dataset.csv** dosyasını pandas kütüphanesi aracılığıyla yüklüyoruz ve data adlı değişkene atıyoruz. Bu veri kümesi, spam ve normal e-postaların etiketlendiği bir tablo şeklinde olmalıdır.<br>
 ![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/2eb60e87-37ea-4cdb-b032-22acc0d5556e)
 

**3.**	Veri kümesini özellikler (X) ve hedef değişken (y) olarak ayırıyoruz. text sütununu özellikler olarak, label sütununu ise hedef değişken olarak kullanıyoruz.
 ![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/fb05c741-88ff-40f7-bf4d-ce55f3669375)

**4**.	Veri kümesini eğitim ve test alt kümelerine bölmek için train_test_split fonksiyonunu kullanıyoruz. Burada, X ve y'yi X_train, X_test, y_train ve y_test olarak böldük. Test veri kümesi, toplam veri kümesinin %20'sini oluşturacak şekilde belirledik.
 ![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/95eadd4e-7e6b-4c75-8864-1720b5b377a5)
**5**.	Metin verilerini sayısal vektörlere dönüştürmek için TfidfVectorizer'ı kullanıyoruz. Bu, metin verilerini TF-IDF (Terim Frekansı-Ters Belge Frekansı) değerlerine dönüştürmek için kullanılan bir tekniktir. X_train ve X_test verilerini ayrı ayrı vektörlere dönüştürüyoruz.
 
 ![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/a7e65413-d8e1-4680-b6e3-7817d919d3e2)

**6**.	SVM (Support Vector Machine) modelini oluşturup eğitiyoruz. SVC sınıfını kullanarak varsayılan parametrelerle bir SVM modeli oluşturuyoruz. Eğitim veri kümesini (X_train_vectorized ve y_train) kullanarak modeli eğitiyoruz.
 ![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/37286ce6-842d-4ebb-8362-d6b913711aa4)

**7**.	Modeli değerlendiriyoruz. Test veri kümesini (X_test_vectorized ve y_test) kullanarak tahminler yaparak bir sınıflandırma raporu yazdırıyoruz.
 
![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/868a1b37-c31e-478b-b621-432cfa0a4860)


Bu kod, SVM modelinin performansını değerlendirmek için test veri kümesi üzerinde tahminler yapar ve bir sınıflandırma raporu yazdırır. y_pred = svm_model.predict(X_test_vectorized) satırı, eğitilmiş SVM modelini kullanarak X_test_vectorized veri kümesi üzerinde tahminler yapar ve bu tahminleri y_pred değişkenine atar.
classification_report(y_test, y_pred) satırı, y_test (gerçek etiketler) ve y_pred (tahmin edilen etiketler) arasındaki karşılaştırmayı yapar ve sınıflandırma performansını ayrıntılı bir rapor şeklinde yazdırır. Bu rapor, sınıflandırma modelinin her bir sınıf için precision (kesinlik), recall (duyarlılık), f1-score (F1 skoru) ve support (destek) değerlerini içerir.
Bu sınıflandırma raporu, modelin her bir sınıf için performansını değerlendirmenizi sağlar. Precision, recall ve F1 skorları ne kadar yüksek olursa, modelin o sınıfı doğru bir şekilde sınıflandırma yeteneği o kadar iyidir. Support değeri, veri kümesindeki sınıf dengesini gösterir. <br>
Tablodaki verilere göre yorumlayacak olursak; 
-	Ham sınıfı için precision değeri 1.00'dır, yani modelin ham olarak tahmin ettiği örneklerin tamamı gerçekten hamdır.
-	Spam sınıfı için precision değeri 0.96'dır, yani modelin spam olarak tahmin ettiği örneklerin %96'sı gerçekten spamdır.
-	"Recall" (duyarlılık), gerçekten ham veya spam olan örneklerin model tarafından ne kadar doğru bir şekilde tespit edildiğini gösterir. Ham sınıfı için recall değeri 0.98'dir, yani gerçekten ham olan örneklerin %98'i doğru bir şekilde tespit edilmiştir. Spam sınıfı için recall değeri 0.99'dur, yani gerçekten spam olan örneklerin %99'u doğru bir şekilde tespit edilmiştir.
-	"F1-score" ise precision ve recall değerlerinin harmonik ortalamasını temsil eder. Bu metrik, modelin hem precision hem de recall açısından ne kadar iyi performans gösterdiğini gösterir. Ham sınıfı için F1-score değeri 0.99'dur, yani modelin hem precision hem de recall açısından ham sınıfı iyi bir şekilde tahmin ettiğini gösterir. Spam sınıfı için F1-score değeri 0.97'dir, yani modelin hem precision hem de recall açısından spam sınıfını iyi bir şekilde tahmin ettiğini gösterir. 
-	"Support" değeri, her bir sınıf için veri kümesindeki örnek sayısını temsil eder. Bu tabloda, ham sınıfı için 742 örnek ve spam sınıfı için 293 örnek bulunmaktadır.
-	Son olarak, "accuracy" (doğruluk) değeri, modelin doğru sınıflandırma oranını gösterir. Bu tabloda, 1035 örneğin %99'u doğru bir şekilde sınıflandırılmıştır.
Genel olarak, bu tablo modelin spam mailleri filtrelemedeki performansının oldukça yüksek olduğunu göstermektedir. Hem ham hem de spam sınıfı için yüksek precision, recall ve F1-score değerleri, modelin hem doğru ham tahminler yapabildiğini hem de spam maillerini başarılı bir şekilde tespit edebildiğini gösterir. Accuracy değeri de yüksek olduğundan, modelin genel olarak iyi bir performansa sahip olduğunu söyleyebiliriz. <br>


**8**.	Demo uygulamasını başlatıyoruz. Bir döngü içinde kullanıcıdan bir e-posta metni alıyoruz (user_input). Kullanıcının girdisi olan metni, vektörize etmek için vectorizer'ı kullanıyoruz. Ardından, SVM modeliyle (svm_model) tahmin yaparak sonucu alıyoruz. Eğer sonuç 'spam' ise kullanıcıya "Bu bir spam e-postasıdır." mesajını gösteriyoruz, aksi takdirde "Bu bir normal e-postadır." mesajını gösteriyoruz.<br>
![image](https://github.com/KardelRuveyda/detect-email-spam-vs-ham/assets/33912144/73614e02-9fec-4e8f-912b-84ce3a71be71)

