package cz.cvut.swa.bazaar.productservice.data

import java.math.BigDecimal
import java.time.LocalDateTime

data class ProductDTO(

        val title: String,
        val description: String,
        val price: BigDecimal,
        val sellerId: String

)

data class ProductStatusUpdateDTO(

        val id: String,
        val newStatus: ProductStatus

)

data class ProductWithReviewsDTO (

        val id: String,
        val title: String,
        val description: String,
        val price: BigDecimal,
        val sellerId: String,
        var status: ProductStatus,
        val postedDatetime: LocalDateTime,

        var reviews: List<ReviewDTO>

)

data class ReviewDTO(

        val id: String,
        val reviewerId: String,
        val comment: String,
        val reviewedDatetime: LocalDateTime

)