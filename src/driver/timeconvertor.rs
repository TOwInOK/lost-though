use std::io::Error;
use chrono::NaiveDateTime;
use mongodb::bson::DateTime as BsonDateTime;
#[allow(unused)]
pub async fn to_timestamp(input: BsonDateTime) -> Result<i64, Error> {
    if let Some(time) = NaiveDateTime::from_timestamp_millis(input.timestamp_millis()) {
        let output = time.timestamp_millis();
        Ok(output)
    } else {
        Err(Error::new(std::io::ErrorKind::NotFound, "Failed to convert BsonDateTime to DateTime"))
    }
}
#[allow(unused)]
pub async fn to_bson_date_time (input: i64) -> Result<BsonDateTime, Error> {
    Ok(BsonDateTime::from_millis(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_to_timestamp() {
        // Тест для функции to_timestamp
        let bson_datetime = BsonDateTime::from_millis(1635685200000); // Пример BsonDateTime

        // Вызываем функцию и ожидаем успешный результат
        let result = to_timestamp(bson_datetime).await;
        assert!(result.is_ok());

        // Распаковываем результат и проверяем, соответствует ли он ожидаемому значению
        let timestamp = result.unwrap();
        assert_eq!(timestamp, 1635685200000);
    }

    #[tokio::test]
    async fn test_to_bson_date_time() {
        // Тест для функции to_BsonDateTime
        let input_timestamp = 1635685200000; // Пример временной метки

        // Вызываем функцию и ожидаем успешный результат
        let result = to_bson_date_time(input_timestamp).await;
        assert!(result.is_ok());

        // Распаковываем результат и проверяем, является ли он эквивалентом входному значению
        let bson_datetime = result.unwrap();
        assert_eq!(bson_datetime.timestamp_millis(), input_timestamp);
    }
}
