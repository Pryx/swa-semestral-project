package cz.cvut.swa.bazaar.productservice.data

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.repository.MongoRepository
import java.time.LocalDateTime

data class Review(

        @Id val id: String,
        var productId: String,
        val reviewerId: String,
        val text: String,
        val rating: Float? = null,
        val reviewedDatetime: LocalDateTime = LocalDateTime.now()

)

interface ReviewRepository : MongoRepository<Review, String>